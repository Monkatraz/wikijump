<?php

namespace Wikidot\DB;




use Ozone\Framework\Database\BaseDBPeer;

/**
 * Base peer Class mapped to the database table form_submission_key.
 */
class FormSubmissionKeyPeerBase extends BaseDBPeer
{
    public static $peerInstance;

    protected function internalInit()
    {
        $this->tableName='form_submission_key';
        $this->objectName='Wikidot\\DB\\FormSubmissionKey';
        $this->primaryKeyName = 'key_id';
        $this->fieldNames = array( 'key_id' ,  'date_submitted' );
        $this->fieldTypes = array( 'key_id' => 'varchar(90)',  'date_submitted' => 'timestamp');
        $this->defaultValues = array();
    }

    public static function instance()
    {
        if (self::$peerInstance == null) {
            $className = 'Wikidot\\DB\\FormSubmissionKeyPeer';
            self::$peerInstance = new $className();
        }
        return self::$peerInstance;
    }
}
