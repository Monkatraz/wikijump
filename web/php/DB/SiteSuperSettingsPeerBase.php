<?php

namespace Wikidot\DB;




use Ozone\Framework\Database\BaseDBPeer;

/**
 * Base peer Class mapped to the database table site_super_settings.
 */
class SiteSuperSettingsPeerBase extends BaseDBPeer
{
    public static $peerInstance;

    protected function internalInit()
    {
        $this->tableName='site_super_settings';
        $this->objectName='Wikidot\\DB\\SiteSuperSettings';
        $this->primaryKeyName = 'site_id';
        $this->fieldNames = array( 'site_id' ,  'can_custom_domain' );
        $this->fieldTypes = array( 'site_id' => 'int',  'can_custom_domain' => 'boolean');
        $this->defaultValues = array( 'can_custom_domain' => 'false');
    }

    public static function instance()
    {
        if (self::$peerInstance == null) {
            $className = 'Wikidot\\DB\\SiteSuperSettingsPeer';
            self::$peerInstance = new $className();
        }
        return self::$peerInstance;
    }
}
