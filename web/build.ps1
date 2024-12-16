param(
    [Parameter(Mandatory)]
    [string]$Site
)

switch ($Site) {
    "features" {
        Push-Location features

        npm ci
        npm run build

        Pop-Location
    }
    default {
        Write-Error "Unknown site: $Site"
        exit 1
    }
}
