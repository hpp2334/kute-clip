import path from 'path'
import pkg from '../package.json' with { type: "json" }
import { ROOT, DIR_ARTIFACTS } from './base.js'
import { mkdirSync, copyFileSync, cpSync, existsSync, rmSync } from 'fs'
import { execSync } from 'child_process'

execSync("pnpm tauri build --ci", {
    cwd: ROOT,
    stdio: 'inherit'
})

mkdirSync(DIR_ARTIFACTS, { recursive: true })

const outputApp = path.resolve(DIR_ARTIFACTS, './KuteClip.app')
const outputDmg = path.resolve(DIR_ARTIFACTS, './KuteClip.dmg')

if (existsSync(outputApp)) {
    rmSync(outputApp, { recursive: true, force: true })
}
if (existsSync(outputDmg)) {
    rmSync(outputDmg, { recursive: true, force: true })
}

cpSync(
    path.resolve(ROOT, './src-tauri/target/release/bundle/macos/KuteClip.app'),
    outputApp,
    {recursive: true}
)
copyFileSync(
    path.resolve(ROOT, `./src-tauri/target/release/bundle/dmg/KuteClip_${pkg.version}_x64.dmg`),
    outputDmg
)