#!/usr/bin/env node

import https from 'https'
import { exec } from 'child_process'

const applySettings = settings => {
    exec('gsettings get org.gnome.Terminal.ProfilesList default | sed s/\\\'//g', (error, stdout, stderr) => {
        const profile = stdout.trim()
        const gsettingsPath = `org.gnome.Terminal.Legacy.Profile:/org/gnome/terminal/legacy/profiles:/:${profile}/`
        settings.forEach((value, key) => exec(`gsettings set ${gsettingsPath} ${key} "${value}"`))
        console.log(`Applied settings to GNOME Terminal profile ${profile}.`)
    });
    exec('gsettings get com.gexperts.Tilix.ProfilesList default | sed s/\\\'//g', (error, stdout, stderr) => {
        const profile = stdout.trim()
        const gsettingsPath = `com.gexperts.Tilix.Profile:/com/gexperts/Tilix/profiles/${profile}/`
        settings.forEach((value, key) => exec(`gsettings set ${gsettingsPath} ${key} "${value}"`))
        console.log(`Applied settings to Tilix profile ${profile}.`)
    });
}

if (process.argv.length < 3) {
    console.error(`Usage: van <theme_name>
Example: van elementary
See https://mayccoll.github.io/Gogh/ for theme names.`)
    process.exit(1)
}
const themeName = process.argv[2]

https.get('https://raw.githubusercontent.com/Mayccoll/Gogh/master/data/themes.json', res => {
    let body = '';
    res.on('data', chunk => {
        body += chunk
    })
    res.on('end', () => {
        const theme = JSON.parse(body)['themes'].filter(theme => theme['name'].toLowerCase() == themeName.toLowerCase())[0]
        if (!theme) {
            console.log(`Couldn't find theme '${themeName}'.`)
            process.exit(1)
        }
        const { name, background, foreground, ...colors } = theme
        const settings = new Map([
            ['background-color', background],
            ['foreground-color', foreground],
            ['palette', `[${Object.values(colors).map(value => value).join(', ')}]`]
        ])
        applySettings(settings)
    });
})