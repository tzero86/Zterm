#!/usr/bin/env powershell
#
# Rebrand script: replaces all Warp references with Zterm throughout the codebase.
# Run from the repo root: powershell -ExecutionPolicy Bypass ./script/rebrand.ps1

$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
Set-Location $repoRoot

Write-Host "=== Zterm Rebrand Script ===" -ForegroundColor Cyan
Write-Host "Repo root: $repoRoot"

# ---------------------------------------------------------------------------
# Helper: replace text in a file (UTF-8, preserve line endings)
# ---------------------------------------------------------------------------
function Replace-InFile {
    param([string]$filePath, [string]$oldText, [string]$newText)
    $raw = [System.IO.File]::ReadAllText($filePath)
    if ($raw.Contains($oldText)) {
        $updated = $raw.Replace($oldText, $newText)
        [System.IO.File]::WriteAllText($filePath, $updated, [System.Text.Encoding]::UTF8)
        return $true
    }
    return $false
}

# ---------------------------------------------------------------------------
# Collect all text files to process (skip .git, target, Cargo.lock)
# ---------------------------------------------------------------------------
$extensions = @('*.rs','*.toml','*.md','*.sh','*.ps1','*.py','*.json','*.hbs',
                 '*.yml','*.yaml','*.svg','*.plist','*.iss','*.txt','*.gitignore',
                 '*.gitattributes','*.clang-format','*.clippy.toml','*.wgsl',
                 '*.graphql','*.sql','*.html','*.css','*.scss','*.ts','*.js',
                 '*.psd1','*.psm1','*.diesel')

$allFiles = @()
foreach ($ext in $extensions) {
    $allFiles += Get-ChildItem -Recurse -File -Filter $ext -Path $repoRoot |
        Where-Object { $_.FullName -notmatch '\\\.git\\' -and $_.FullName -notmatch '\\target\\' }
}

# Also include extension-less scripts in ./script/
$allFiles += Get-ChildItem -File -Path "$repoRoot\script" |
    Where-Object { $_.Extension -eq '' -and $_.FullName -notmatch '\\\.git\\' }
$allFiles += Get-ChildItem -File -Path "$repoRoot\script\windows" |
    Where-Object { $_.Extension -eq '' -and $_.FullName -notmatch '\\\.git\\' }
$allFiles += Get-ChildItem -File -Path "$repoRoot\script\linux" -ErrorAction SilentlyContinue |
    Where-Object { $_.Extension -eq '' -and $_.FullName -notmatch '\\\.git\\' }
$allFiles += Get-ChildItem -File -Path "$repoRoot\script\macos" -ErrorAction SilentlyContinue |
    Where-Object { $_.Extension -eq '' -and $_.FullName -notmatch '\\\.git\\' }

# Deduplicate
$allFiles = $allFiles | Sort-Object FullName -Unique

Write-Host "Found $($allFiles.Count) files to process."

# ---------------------------------------------------------------------------
# Ordered replacement table  (most-specific / longest strings FIRST)
# Using array of pairs because PowerShell hashtables are case-insensitive
# and some keys differ only by case (e.g. 'WarpOss' vs 'warposs').
# ---------------------------------------------------------------------------
$replacements = @(
    # Internal crate names (compound warpui* before warpui)
    ,('warpui_core',               'zterm_ui_core')
    ,('warpui_extras',             'zterm_ui_extras')
    ,('warpui',                    'zterm_ui')

    # Internal warp_* crate names
    ,('warp_cli',                  'zterm_cli')
    ,('warp_completer',            'zterm_completer')
    ,('warp_core',                 'zterm_core')
    ,('warp_features',             'zterm_features')
    ,('warp_files',                'zterm_files')
    ,('warp_graphql_schema',       'zterm_graphql_schema')
    ,('warp_js',                   'zterm_js')
    ,('warp_logging',              'zterm_logging')
    ,('warp_ripgrep',              'zterm_ripgrep')
    ,('warp_server_client',        'zterm_server_client')
    ,('warp_terminal',             'zterm_terminal')
    ,('warp_util',                 'zterm_util')
    ,('warp_web_event_bus',        'zterm_web_event_bus')

    # Binary / channel config binary
    ,('warp-channel-config',       'zterm-channel-config')
    ,('warp-oss',                  'zterm-oss')

    # URL schemes (longer strings before shorter)
    ,('warppreview',               'ztermpreview')
    ,('warpstable',                'ztermstable')
    ,('warplocal',                 'ztermlocal')
    ,('warposs',                   'ztermoss')
    ,('warpdev',                   'ztermdev')

    # Bundle/app identifiers
    ,('dev.warp.',                 'dev.zterm.')
    ,('com.warp.',                 'com.zterm.')

    # Environment variables (ALL CAPS)
    ,('WARP_API_KEY',              'ZTERM_API_KEY')
    ,('WARP_APP_NAME',             'ZTERM_APP_NAME')
    ,('WARP_CLOUD_MODE_DEFAULT_HOST', 'ZTERM_CLOUD_MODE_DEFAULT_HOST')
    ,('WARP_DRIVE_SYNC_TIMEOUT',   'ZTERM_DRIVE_SYNC_TIMEOUT')
    ,('WARP_CHANNEL',              'ZTERM_CHANNEL')
    ,('WARP_BIN_NAME',             'ZTERM_BIN_NAME')
    ,('WARP_ARGS',                 'ZTERM_ARGS')

    # Proper-case product name compounds (before generic Warp)
    ,('WarpOss',                   'ZtermOss')
    ,('WarpDev',                   'ZtermDev')
    ,('WarpLocal',                 'ZtermLocal')
    ,('WarpPreview',               'ZtermPreview')
    ,('WarpStable',                'ZtermStable')
    ,('WarpUI',                    'ZtermUI')
    ,('Warp Team',                 'Zterm Team')
    ,('Warp Drive',                'Zterm Drive')
    ,('Warp ADE',                  'Zterm ADE')

    # Main app crate (lib name) references in source
    ,('use warp::',                'use zterm::')
    ,('pub use warp::',            'pub use zterm::')
    ,('warp::run()',               'zterm::run()')
    ,('= warp::',                  '= zterm::')

    # Cargo.toml package/dep name for the main app crate
    ,('name = "warp"',             'name = "zterm"')
    ,('warp = { path = "app"',     'zterm = { path = "app"')
    ,('warp = { workspace = true', 'zterm = { workspace = true')
    ,('lib-name = "warp"',         'lib-name = "zterm"')
    ,('default-run = "warp"',      'default-run = "zterm"')

    # Remaining WARP_ (catches any we didn't enumerate above)
    ,('WARP_',                     'ZTERM_')

    # Generic product name (do AFTER all compound forms)
    ,('"Warp"',                    '"Zterm"')
    ,("'Warp'",                    "'Zterm'")
    ,('# Warp',                    '# Zterm')
    ,('// Warp',                   '// Zterm')
    ,('for Warp',                  'for Zterm')
    ,('of Warp',                   'of Zterm')
    ,('run Warp',                  'run Zterm')
    ,('from Warp',                 'from Zterm')
    ,('with Warp',                 'with Zterm')
    ,('build Warp',                'build Zterm')
    ,('local Warp',                'local Zterm')
    ,('using Warp',                'using Zterm')
    ,('Install Warp',              'Install Zterm')
    ,('Launch Warp',               'Launch Zterm')
)

# ---------------------------------------------------------------------------
# Apply replacements to all files
# ---------------------------------------------------------------------------
$changedFiles = 0
foreach ($file in $allFiles) {
    $fileChanged = $false
    foreach ($pair in $replacements) {
            $changed = Replace-InFile -filePath $file.FullName -oldText $pair[0] -newText $pair[1]
        if ($changed) { $fileChanged = $true }
    }
    if ($fileChanged) {
        $changedFiles++
        Write-Host "  Updated: $($file.FullName.Replace($repoRoot, ''))"
    }
}
Write-Host ""
Write-Host "Text replacements done: $changedFiles files modified." -ForegroundColor Green

# ---------------------------------------------------------------------------
# Phase 2: Rename crate directories
# ---------------------------------------------------------------------------
Write-Host ""
Write-Host "=== Renaming crate directories ===" -ForegroundColor Cyan

$dirRenames = [ordered]@{
    'crates\warpui_core'         = 'crates\zterm_ui_core'
    'crates\warpui_extras'       = 'crates\zterm_ui_extras'
    'crates\warpui'              = 'crates\zterm_ui'
    'crates\warp_cli'            = 'crates\zterm_cli'
    'crates\warp_completer'      = 'crates\zterm_completer'
    'crates\warp_core'           = 'crates\zterm_core'
    'crates\warp_features'       = 'crates\zterm_features'
    'crates\warp_files'          = 'crates\zterm_files'
    'crates\warp_graphql_schema' = 'crates\zterm_graphql_schema'
    'crates\warp_js'             = 'crates\zterm_js'
    'crates\warp_logging'        = 'crates\zterm_logging'
    'crates\warp_ripgrep'        = 'crates\zterm_ripgrep'
    'crates\warp_server_client'  = 'crates\zterm_server_client'
    'crates\warp_terminal'       = 'crates\zterm_terminal'
    'crates\warp_util'           = 'crates\zterm_util'
    'crates\warp_web_event_bus'  = 'crates\zterm_web_event_bus'
}

foreach ($pair in $dirRenames.GetEnumerator()) {
    $src = Join-Path $repoRoot $pair.Key
    $dst = Join-Path $repoRoot $pair.Value
    if (Test-Path $src) {
        Rename-Item -Path $src -NewName (Split-Path $dst -Leaf)
        Write-Host "  Renamed: $($pair.Key) -> $($pair.Value)"
    } else {
        Write-Host "  (skipped, not found): $($pair.Key)" -ForegroundColor Yellow
    }
}

# ---------------------------------------------------------------------------
# Phase 3: Rename individual files
# ---------------------------------------------------------------------------
Write-Host ""
Write-Host "=== Renaming files ===" -ForegroundColor Cyan

$fileRenames = [ordered]@{
    'WARP.md'               = 'ZTERM.md'
    '.warpindexingignore'   = '.ztermindexingignore'
    'script\warp.svg'       = 'script\zterm.svg'
}

foreach ($pair in $fileRenames.GetEnumerator()) {
    $src = Join-Path $repoRoot $pair.Key
    $dst = Join-Path $repoRoot $pair.Value
    if (Test-Path $src) {
        Rename-Item -Path $src -NewName (Split-Path $dst -Leaf)
        Write-Host "  Renamed: $($pair.Key) -> $($pair.Value)"
    } else {
        Write-Host "  (skipped, not found): $($pair.Key)" -ForegroundColor Yellow
    }
}

# ---------------------------------------------------------------------------
# Phase 4: Rename .warp directory -> .zterm
# ---------------------------------------------------------------------------
Write-Host ""
Write-Host "=== Renaming .warp directory ===" -ForegroundColor Cyan

$warpDir = Join-Path $repoRoot '.warp'
$ztermDir = Join-Path $repoRoot '.zterm'
if (Test-Path $warpDir) {
    Rename-Item -Path $warpDir -NewName '.zterm'
    Write-Host "  Renamed: .warp -> .zterm"
} else {
    Write-Host "  (skipped, .warp not found)" -ForegroundColor Yellow
}

# ---------------------------------------------------------------------------
# Phase 5: Update Cargo.toml workspace members paths
# ---------------------------------------------------------------------------
Write-Host ""
Write-Host "=== Updating workspace member paths in Cargo.toml ===" -ForegroundColor Cyan

$cargoToml = Join-Path $repoRoot 'Cargo.toml'
$raw = [System.IO.File]::ReadAllText($cargoToml)
# Replace all remaining crates/warp_* and crates/warpui* paths
$raw = $raw -replace 'crates/warpui_core', 'crates/zterm_ui_core'
$raw = $raw -replace 'crates/warpui_extras', 'crates/zterm_ui_extras'
$raw = $raw -replace 'crates/warpui', 'crates/zterm_ui'
$raw = $raw -replace 'crates/warp_cli', 'crates/zterm_cli'
$raw = $raw -replace 'crates/warp_completer', 'crates/zterm_completer'
$raw = $raw -replace 'crates/warp_core', 'crates/zterm_core'
$raw = $raw -replace 'crates/warp_features', 'crates/zterm_features'
$raw = $raw -replace 'crates/warp_files', 'crates/zterm_files'
$raw = $raw -replace 'crates/warp_graphql_schema', 'crates/zterm_graphql_schema'
$raw = $raw -replace 'crates/warp_js', 'crates/zterm_js'
$raw = $raw -replace 'crates/warp_logging', 'crates/zterm_logging'
$raw = $raw -replace 'crates/warp_ripgrep', 'crates/zterm_ripgrep'
$raw = $raw -replace 'crates/warp_server_client', 'crates/zterm_server_client'
$raw = $raw -replace 'crates/warp_terminal', 'crates/zterm_terminal'
$raw = $raw -replace 'crates/warp_util', 'crates/zterm_util'
$raw = $raw -replace 'crates/warp_web_event_bus', 'crates/zterm_web_event_bus'
[System.IO.File]::WriteAllText($cargoToml, $raw, [System.Text.Encoding]::UTF8)
Write-Host "  Updated Cargo.toml workspace paths"

# Also update all per-crate Cargo.toml path references
$allCargoTomls = Get-ChildItem -Recurse -File -Filter 'Cargo.toml' -Path $repoRoot |
    Where-Object { $_.FullName -notmatch '\\\.git\\' -and $_.FullName -notmatch '\\target\\' }
foreach ($ct in $allCargoTomls) {
    $raw = [System.IO.File]::ReadAllText($ct.FullName)
    $updated = $raw `
        -replace 'crates/warpui_core', 'crates/zterm_ui_core' `
        -replace 'crates/warpui_extras', 'crates/zterm_ui_extras' `
        -replace 'crates/warpui', 'crates/zterm_ui' `
        -replace 'crates/warp_cli', 'crates/zterm_cli' `
        -replace 'crates/warp_completer', 'crates/zterm_completer' `
        -replace 'crates/warp_core', 'crates/zterm_core' `
        -replace 'crates/warp_features', 'crates/zterm_features' `
        -replace 'crates/warp_files', 'crates/zterm_files' `
        -replace 'crates/warp_graphql_schema', 'crates/zterm_graphql_schema' `
        -replace 'crates/warp_js', 'crates/zterm_js' `
        -replace 'crates/warp_logging', 'crates/zterm_logging' `
        -replace 'crates/warp_ripgrep', 'crates/zterm_ripgrep' `
        -replace 'crates/warp_server_client', 'crates/zterm_server_client' `
        -replace 'crates/warp_terminal', 'crates/zterm_terminal' `
        -replace 'crates/warp_util', 'crates/zterm_util' `
        -replace 'crates/warp_web_event_bus', 'crates/zterm_web_event_bus'
    if ($updated -ne $raw) {
        [System.IO.File]::WriteAllText($ct.FullName, $updated, [System.Text.Encoding]::UTF8)
    }
}
Write-Host "  Updated per-crate Cargo.toml path references"

Write-Host ""
Write-Host "=== Rebrand complete! ===" -ForegroundColor Green
Write-Host "Next steps:"
Write-Host "  1. Review changes with: git diff"
Write-Host "  2. Build to verify: cargo check --bin zterm-oss --features gui"
Write-Host "  3. Fix any remaining compile errors"
