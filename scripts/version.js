import path from 'path'
import { ROOT } from './base.js'
import { readFileSync, writeFileSync } from 'fs'

const nextVersion = process.argv.slice(-1)[0]
if (!nextVersion || !/^\d+\.\d+\.\d+$/.test(nextVersion)) {
    throw Error(`version "${nextVersion}" is invalid`);
}

const pkgJsonPath = path.resolve(ROOT, './package.json')
const tauriJsonPath = path.resolve(ROOT, './src-tauri/tauri.conf.json')

for (const p of [pkgJsonPath, tauriJsonPath]) {
    const s = readFileSync(p, 'utf-8')
    const o = JSON.parse(s)
    o.version = nextVersion
    writeFileSync(p, JSON.stringify(o, null, 4))
}