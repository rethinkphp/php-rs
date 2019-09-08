<?php

use PHPUnit\Framework\TestCase;

class HelloWorldTest extends TestCase 
{
    public function testSayHello()
    {
        ob_start();
        $result = say_hello("Rust");
        $this->assertEquals('Hello world, Rust!', ob_get_clean());
        $this->assertEquals('Rust', $result);
    }
}