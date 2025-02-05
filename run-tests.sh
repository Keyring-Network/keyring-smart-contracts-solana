#!/bin/bash

# Exit on any error
set -e

anchor build

anchor test test_initialize --skip-build
anchor test test_register_key --skip-build
anchor test test_revoke_key --skip-build
anchor test test_set_admin --skip-build
anchor test test_blacklist_entity --skip-build
anchor test test_unblacklist_entity --skip-build
anchor test test_create_credentials --skip-build
anchor test test_collect_fees --skip-build