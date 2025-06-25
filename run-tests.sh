#!/bin/bash

# Exit on any error
set -e

anchor build

# We are waiting for 5 seconds between test for test validator to shut down properly

anchor test test_initialize --skip-build
sleep 5s

anchor test test_register_key --skip-build
sleep 5s

anchor test test_revoke_key --skip-build
sleep 5s

anchor test test_manage_roles --skip-build
sleep 5s

anchor test test_blacklist_entity --skip-build
sleep 5s

anchor test test_unblacklist_entity --skip-build
sleep 5s

anchor test test_create_credentials --skip-build
sleep 5s

anchor test test_collect_fees --skip-build
sleep 5s

anchor test test_check_credential --skip-build
sleep 5s

anchor test test_verify_auth_message --skip-build
