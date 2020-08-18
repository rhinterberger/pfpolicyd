# pfpolicyd - Simple Postfix policy daemon

## Features 
* Accounting and Quotas per sasl_username

## Missing and future Features

## Installation
_Setup Database_

_How to Integrate to systemd_

## Usage
_How to use this_

## Theory of Operation

pfpolicyd runs as service and listens for connections on a tcp port. It takes requests from Postfix via check_policy_service as defined in http://www.postfix.org/SMTPD_POLICY_README.html

It looks up the current and maximum quotas in the backend database with sasl_username as key. On current_quota < default_quota a accept response is sent back to Postfix, if quotas are exceeded, a defer response is sent. Values for daily and monthly quotas are incremented.

If there are no entries found in the database, an entry with default values will be automatically created. Current default values are 200 mails per day and 2000 per month.

At the moment database entries for current_quota need do be reset manually or with a cron-job every day and on the 1st of every month.

Every Postfix-connection is handled in a seperate Thread. You can connect multiple Postfixservers to a single instance of pfpolicyd.