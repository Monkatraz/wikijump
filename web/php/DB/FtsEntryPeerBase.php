<?php

namespace Wikidot\DB;




use Ozone\Framework\Database\BaseDBPeer;

/**
 * Base peer Class mapped to the database table fts_entry.
 */
class FtsEntryPeerBase extends BaseDBPeer
{
    public static $peerInstance;

    protected function internalInit()
    {
        $this->tableName='fts_entry';
        $this->objectName='Wikidot\\DB\\FtsEntry';
        $this->primaryKeyName = 'fts_id';
        $this->fieldNames = array( 'fts_id' ,  'page_id' ,  'title' ,  'unix_name' ,  'thread_id' ,  'site_id' ,  'text' ,  'vector' );
        $this->fieldTypes = array( 'fts_id' => 'serial',  'page_id' => 'int',  'title' => 'varchar(256)',  'unix_name' => 'varchar(100)',  'thread_id' => 'int',  'site_id' => 'int',  'text' => 'text',  'vector' => 'tsvector');
        $this->defaultValues = array();
    }

    public static function instance()
    {
        if (self::$peerInstance == null) {
            $className = 'Wikidot\\DB\\FtsEntryPeer';
            self::$peerInstance = new $className();
        }
        return self::$peerInstance;
    }
}
