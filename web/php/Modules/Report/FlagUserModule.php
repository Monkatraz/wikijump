<?php

namespace Wikidot\Modules\Report;

use Ozone\Framework\Database\Criteria;
use Wikidot\DB\OzoneUserPeer;
use Wikidot\DB\UserAbuseFlagPeer;

use Ozone\Framework\SmartyModule;
use Wikidot\Utils\ProcessException;
use Wikidot\Utils\WDPermissionException;

class FlagUserModule extends SmartyModule
{

    public function isAllowed($runData)
    {
        $userId = $runData->getUserId();
        if ($userId == null || $userId <1) {
            throw new WDPermissionException(_("This option is available only to registered (and logged-in) users."));
        }
        return true;
    }

    public function build($runData)
    {
        $pl = $runData->getParameterList();

        $targetUserId = $pl->getParameterValue("targetUserId");
        if ($targetUserId == null || $targetUserId == '' || !is_numeric($targetUserId)) {
            throw new ProcessException(_("Error processing the request."), "no_target_user");
        }

        $targetUser = OzoneUserPeer::instance()->selectByPrimaryKey($targetUserId);
        if ($targetUser == null) {
            throw new ProcessException(_("Error processing the request."), "no_target_user");
        }

        $site = $runData->getTemp("site");
        $user = $runData->getUser();

        if ($targetUser->getUserId() === $user->getUserId()) {
            throw new ProcessException(_("You cannot flag yourself as an abusive user."), "not_yourself");
        }

        // check if flagged already
        $c = new Criteria();
        $c->add("user_id", $user->getUserId());
        $c->add("target_user_id", $targetUser->getUserId());

        $flag = UserAbuseFlagPeer::instance()->selectOne($c);

        if ($flag) {
            $runData->contextAdd("flagged", true);
        }

        $runData->contextAdd("user", $targetUser);
    }
}
