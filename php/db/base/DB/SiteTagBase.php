<?php


namespace DB;

use BaseDBObject;

/**
 * Base class mapped to the database table site_tag.
 */
class SiteTagBase extends BaseDBObject
{

    protected function internalInit()
    {
        $this->tableName='site_tag';
        $this->peerName = 'DB\\SiteTagPeer';
        $this->primaryKeyName = 'tag_id';
        $this->fieldNames = array( 'tag_id' ,  'site_id' ,  'tag' );

        //$this->fieldDefaultValues=
    }






    public function getTagId()
    {
        return $this->getFieldValue('tag_id');
    }

    public function setTagId($v1, $raw = false)
    {
        $this->setFieldValue('tag_id', $v1, $raw);
    }


    public function getSiteId()
    {
        return $this->getFieldValue('site_id');
    }

    public function setSiteId($v1, $raw = false)
    {
        $this->setFieldValue('site_id', $v1, $raw);
    }


    public function getTag()
    {
        return $this->getFieldValue('tag');
    }

    public function setTag($v1, $raw = false)
    {
        $this->setFieldValue('tag', $v1, $raw);
    }
}
