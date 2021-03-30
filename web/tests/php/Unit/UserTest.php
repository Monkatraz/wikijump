<?php

namespace Tests\Unit;

use Tests\TestCase;
use Wikijump\Models\User;
use Illuminate\Foundation\Testing\DatabaseTransactions;

class UserTest extends TestCase
{
    use DatabaseTransactions;

    private $user;

    public function setUp() : void
    {
        parent::setUp();

        $this->user = User::find(1);
    }

    /**
     * Basic DB interaction.
     *
     * @return void
     */
    public function test_user_table()
    {
        $this->assertEquals('Administrator', $this->user->name);
    }
}
