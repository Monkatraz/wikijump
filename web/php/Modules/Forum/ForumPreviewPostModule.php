<?php

namespace Wikidot\Modules\Forum;


use Ozone\Framework\ODate;
use Wikidot\DB\ForumPost;

use Ozone\Framework\SmartyModule;
use Wikidot\Utils\ProcessException;
use Wikidot\Utils\WikiTransformation;

class ForumPreviewPostModule extends SmartyModule
{

    public function build($runData)
    {
        $pl = $runData->getParameterList();
        $title = $pl->getParameterValue("title");
        $description = trim($pl->getParameterValue("description"));
        $source = trim($pl->getParameterValue("source"));

        if ($source == null || $source == '') {
            throw new ProcessException(_("Post is empty."), "post_empty");
        }

        $wt = new WikiTransformation();
        $wt->setMode('post');
        $body = $wt->processSource($source);

        $post = new ForumPost();

        $post->setText($body);
        $post->setTitle($title);
        $post->setDatePosted(new ODate());

        // now set user_id, user_string

        $userId = $runData->getUserId();
        if ($userId == null) {
            $userString = $runData->createIpString();
        }

        if ($userId) {
            $post->setUserId($userId);
        } else {
            $post->setUserId(0);
            $post->setUserString($userString);
        }

        $runData->contextAdd("post", $post);
    }
}
