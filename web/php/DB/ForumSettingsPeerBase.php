<?php

namespace Wikidot\DB;




use Ozone\Framework\Database\BaseDBPeer;

/**
 * Base peer Class mapped to the database table forum_settings.
 */
class ForumSettingsPeerBase extends BaseDBPeer
{
    public static $peerInstance;

    protected function internalInit()
    {
        $this->tableName='forum_settings';
        $this->objectName='Wikidot\\DB\\ForumSettings';
        $this->primaryKeyName = 'site_id';
        $this->fieldNames = array( 'site_id' ,  'permissions' ,  'per_page_discussion' ,  'max_nest_level' );
        $this->fieldTypes = array( 'site_id' => 'int',  'permissions' => 'varchar(200)',  'per_page_discussion' => 'boolean',  'max_nest_level' => 'int');
        $this->defaultValues = array( 'per_page_discussion' => 'false',  'max_nest_level' => '0');
    }

    public static function instance()
    {
        if (self::$peerInstance == null) {
            $className = 'Wikidot\\DB\\ForumSettingsPeer';
            self::$peerInstance = new $className();
        }
        return self::$peerInstance;
    }
}
