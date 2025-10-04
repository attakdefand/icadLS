# Test the API with a simple request
$json = @{
    code = "use std::collections::HashMap; fn main() { let mut map = HashMap::new(); map.insert(""key"", ""value""); }"
} | ConvertTo-Json

try {
    $response = Invoke-WebRequest -Uri "http://localhost:8081/analyze" -Method POST -ContentType "application/json" -Body $json
    Write-Host "Response Status: $($response.StatusCode)"
    Write-Host "Response Content:"
    Write-Host $response.Content
} catch {
    Write-Host "Error: $($_.Exception.Message)"
}