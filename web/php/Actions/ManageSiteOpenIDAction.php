<?php

namespace Wikidot\Actions;
use Ozone\Framework\Database\Criteria;
use Ozone\Framework\Database\Database;
use Ozone\Framework\JSONService;
use Ozone\Framework\SmartyAction;
use Wikidot\DB\OpenidEntryPeer;
use Wikidot\DB\PagePeer;
use Wikidot\DB\OpenidEntry;
use Wikidot\Utils\GlobalProperties;
use Wikidot\Utils\Outdater;
use Wikidot\Utils\ProcessException;
use Wikidot\Utils\WDPermissionManager;

class ManageSiteOpenIDAction extends SmartyAction
{

    public function isAllowed($runData)
    {
        WDPermissionManager::instance()->hasPermission('manage_site', $runData->getUser(), $runData->getTemp("site"));
        return true;
    }

    public function perform($r)
    {
    }

    public function saveOpenIDEvent($runData)
    {
        $pl = $runData->getParameterList();

        $site = $runData->getTemp("site");
        $settings = $site->getSettings();

        $openIdEnabled = $pl->getParameterValue("enableOpenID") == "true";

        $json = new JSONService(SERVICES_JSON_LOOSE_TYPE);

        $vals = $json->decode($pl->getParameterValue("vals"));

        $affectedPages = array();

        $outdater = new Outdater();

        $db = Database::connection();
        $db->begin();

        $settings->setOpenidEnabled($openIdEnabled);
        $settings->save();

        // get the already assigned openids
        $c = new Criteria();
        $c->add("site_id", $site->getSiteId());
        $oldOpenIDs = OpenidEntryPeer::instance()->select($c);

        $rootProcessed = false;

        $oldOpenIDs2 = $oldOpenIDs;

        foreach ($vals as $val) {
            $pageId = null;
            $page = null;
            if ($val['page']) {
                // not a root page
                $page = PagePeer::instance()->selectByName($site->getSiteId(), $val['page']);
                if (!$page) {
                    throw new ProcessException(sprintf(_("The page %s cannot be found"), $vals['page']));
                }

                $pageId = $page->getPageId();
            } elseif (!$rootProcessed) {
                $rootProcessed = true;
            } elseif ($rootProcessed) {
                continue;
            }

            // validate the data
            $url = $val['identityUrl'];
            $serverUrl = $val['serverUrl'];

            if (!$url) {
                continue;
            }

            if (!preg_match(';^[a-z0-9\-\./#]+$;i', $url)) {
                throw new ProcessException(sprintf("Identity URL %s is not valid.", $url));
            }
            if (!preg_match(';^https?://[a-z0-9\-\./#]+$;i', $serverUrl)) {
                throw new ProcessException(sprintf("Server URL %s is not valid.", $serverUrl));
            }

            // check if the entry already exists
            $entry = null;

            foreach ($oldOpenIDs as $oo) {
                if ($oo->getPageId() === $pageId) {
                    $entry = $oo;
                    foreach ($oldOpenIDs2 as $oo2key => &$oo2) {
                        if ($oo2->getPageId() === $pageId) {
                            $ookey = $oo2key;
                            break;
                        }
                    }

                    unset($oldOpenIDs2[$ookey]);

                    break;
                }
            }

            if (!$entry) {
                $entry = new OpenidEntry();
                $entry->setSiteId($site->getSiteId());
                $entry->setPageId($pageId);
            }

            $entry->setUrl(GlobalProperties::$HTTP_SCHEMA . "://" . $url);
            $entry->setServerUrl($serverUrl);
            // save the entry
            $entry->save();

            // outdate caches
            if ($page) {
                $outdater->outdatePageCache($page);
            } else {
                $outdater->outdatePageCache($site->getDefaultPage());
            }
        }

        // remove unused entries
        foreach ($oldOpenIDs2 as $oo) {
            OpenidEntryPeer::instance()->deleteByPrimaryKey($oo->getOpenidId());
            // outdate caches
            $pageId =  $oo->getPageId();
            if ($pageId) {
                $page = PagePeer::instance()->selectByPrimaryKey($pageId);
            } else {
                $page = $site->getDefaultPage();
            }
            $outdater->outdatePageCache($page);
        }

        $db->commit();
    }
}
