#!/bin/bash
git checkout master &&
git pull &&
git checkout space_efficient &&
git pull &&
git merge --no-edit master &&
git push &&
git checkout fnv_hash &&
git pull &&
git merge --no-edit master &&
git push &&
git checkout rustc_hash_hash &&
git pull &&
git merge --no-edit master &&
git push &&
git checkout hash_brown_hash &&
git pull &&
git merge --no-edit master &&
git push &&
git checkout space_efficient_128 &&
git pull &&
git merge --no-edit space_efficient &&
git push 
git checkout space_efficient_max &&
git pull &&
git merge --no-edit space_efficient &&
git push 