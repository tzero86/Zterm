#!/usr/bin/env powershell
# Fixes remaining warp -> zterm crate name and path references in all Cargo.toml files.
Set-Location (Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path))

$pairs = @(
    ,('warpui_core',         'zterm_ui_core')
    ,('warpui_extras',       'zterm_ui_extras')
    ,('warpui',              'zterm_ui')
    ,('warp_cli',            'zterm_cli')
    ,('warp_completer',      'zterm_completer')
    ,('warp_core',           'zterm_core')
    ,('warp_features',       'zterm_features')
    ,('warp_files',          'zterm_files')
    ,('warp_graphql_schema', 'zterm_graphql_schema')
    ,('warp_js',             'zterm_js')
    ,('warp_logging',        'zterm_logging')
    ,('warp_ripgrep',        'zterm_ripgrep')
    ,('warp_server_client',  'zterm_server_client')
    ,('warp_terminal',       'zterm_terminal')
    ,('warp_util',           'zterm_util')
    ,('warp_web_event_bus',  'zterm_web_event_bus')
    ,('name = "warp"',       'name = "zterm"')
    ,('warp = { path = "app"', 'zterm = { path = "app"')
    ,('warp = { workspace = true', 'zterm = { workspace = true')
    ,('default-run = "warp"',    'default-run = "zterm"')
    ,('Warp Team',           'Zterm Team')
)

$tomls = Get-ChildItem -Recurse -Filter 'Cargo.toml' |
    Where-Object { $_.FullName -notmatch [regex]::Escape('\target\') -and $_.FullName -notmatch '[\\/]target[\\/]' }

foreach ($file in $tomls) {
    $content = [System.IO.File]::ReadAllText($file.FullName)
    $original = $content
    foreach ($p in $pairs) {
        $content = $content.Replace($p[0], $p[1])
    }
    if ($content -ne $original) {
        [System.IO.File]::WriteAllText($file.FullName, $content, [System.Text.UTF8Encoding]::new($false))
        Write-Host "Fixed: $($file.FullName)"
    }
}
Write-Host "Done."
