<?php
use DB\ThemePeer;

class ManageSiteEditCustomThemeModule extends ManageSiteBaseModule
{

    public function build($runData)
    {

        $pl = $runData->getParameterList();

        $site = $runData->getTemp("site");
        $runData->contextAdd("site", $site);

        // now select themes that can be extended

        $c = new Criteria();

        $c->add("custom", false);

        $c->addOrderAscending("sort_index");
        $c->addOrderAscending("name");
        $themes = ThemePeer::instance()->select($c);
        $runData->contextAdd("exthemes", $themes);

        $themeId = $pl->getParameterValue("themeId");
        if ($themeId && is_numeric($themeId)) {
            $theme = ThemePeer::instance()->selectByPrimaryKey($themeId);
            if ($theme== null || $theme->getSiteId() !== $site->getSiteId()) {
                throw new ProcessException(_("Error selecting theme."), "wrong_theme");
            }
            $runData->contextAdd("theme", $theme);
            $dir = WIKIDOT_ROOT."/web/files--sites/".$site->getUnixName()."/theme/".$theme->getUnixName();
            $code = file_get_contents($dir."/style.css");
            $runData->contextAdd("code", $code);
        }
    }
}
