# composer require --dev friendsofphp/php-cs-fixer
# curl https://raw.githubusercontent.com/sane-defaults/phpcs/master/.php_cs > .php_cs
check-style:
	vendor/bin/php-cs-fixer fix -v --dry-run --stop-on-violation addr-osmify.php

# composer require --dev slevomat/coding-standard
check-types:
	vendor/bin/phpcs --standard=vendor/slevomat/coding-standard/SlevomatCodingStandard/ruleset.xml addr-osmify.php
