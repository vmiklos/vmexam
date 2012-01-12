#!/usr/bin/perl -w

use strict;

package Person;

sub new
{
    my $class = shift;
    my $self = {
        firstName => shift,
    };
    bless $self, $class;
    return $self;
}

sub firstName : lvalue {
    my $self = shift;
    $self->{firstName};
}

package main;

my $person = new Person();
$person->firstName = "fooo";
print $person->firstName . "\n";
