<?php


namespace Wikidot\Facade;

use Wikidot\Utils\WDPermissionManager;
use Ozone\Framework\Database\Criteria;
use Wikidot\DB\PagePeer;
use Wikidot\DB\CategoryPeer;

class Site extends Base
{
    /**
     * Get pages from a site
     *
     * Argument array keys:
     *  site: site to get pages from
     *  category: category to get pages from (optional)
     *
     * @param array $args
     * @return array
     */
    public function pages(array $args) : array
    {
        $this->parseArgs($args, array("performer", "site"));

        WDPermissionManager::instance()->canAccessSite($this->performer, $this->site);

        $c = new Criteria();
        $c->add("site_id", $this->site->getSiteId());

        if ($this->category) {
            $c->add("category_id", $this->category->getCategoryId());
        }

        $ret = array();
        foreach (PagePeer::instance()->selectByCriteria($c) as $page) {
            $ret[] = $this->repr($page, "meta");
        }
        return $ret;
    }

    /**
     * Get categories from a site
     *
     * Argument array keys:
     *  site: site to get categories from
     *
     * @param array $args
     * @return array
     */
    public function categories(array $args) : array
    {
        $this->parseArgs($args, array("performer", "site"));

        WDPermissionManager::instance()->canAccessSite($this->performer, $this->site);

        $c = new Criteria();
        $c->add("site_id", $this->site->getSiteId());

        $ret = array();
        foreach (CategoryPeer::instance()->selectByCriteria($c) as $category) {
            $ret[] = $this->repr($category);
        }
        return $ret;
    }
}
