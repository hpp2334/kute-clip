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

const outputNsis = path.resolve(DIR_ARTIFACTS, './KuteClip.exe')

if (existsSync(outputNsis)) {
    rmSync(outputNsis, { recursive: true, force: true })
}

copyFileSync(
    path.resolve(ROOT, `./src-tauri/target/release/bundle/nsis/KuteClip_${pkg.version}_x64-setup.exe`),
    outputNsis
)