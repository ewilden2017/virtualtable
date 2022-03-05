#!/bin/bash

rm -r dist/
npm run build
lessc mainstyle.less dist/mainstyle.css
