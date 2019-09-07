<?php

use PHPUnit\Framework\TestCase;

class HelloWorldTest extends TestCase 
{
    public function testSayHello()
    {
        ob_start();
        say_hello();
        $this->assertEquals('Hello world, Rust!', ob_get_clean());
    }
}