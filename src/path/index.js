import { readdirSync } from 'node:fs';

export const binaries = readdirSync('/usr/bin');
