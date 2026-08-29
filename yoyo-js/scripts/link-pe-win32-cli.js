#!/usr/bin/env node
'use strict';
/** stdin JSON → stdout PE bytes. Asm peer uses JS link_pe_win32 for H_00 byte-equal output. */
const fs = require('fs');
const { linkPeWin32 } = require('../src/platform/pe-builder');

const input = JSON.parse(fs.readFileSync(0, 'utf8'));
const code = Buffer.from(input.code, 'hex');
const data = Buffer.from(input.data || '', 'hex');
const handlerOffsets = input.handlerOffsets || [];
const pe = linkPeWin32(code, data, handlerOffsets);
process.stdout.write(pe);
