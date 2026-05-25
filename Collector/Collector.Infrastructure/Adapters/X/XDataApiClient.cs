using System.Net.Http.Headers;
using System.Text.Json;
using Collector.Core.Errors;
using Collector.Core.Ports.X;
using CSharpFunctionalExtensions;
using Microsoft.AspNetCore.Http;

namespace Collector.Infrastructure.Adapters.X;

public sealed class XDataApiClient(HttpClient client, IHttpContextAccessor httpContextAccessor) : IXDataApiClient
{
    private const string GetMyUserEndpoint = "/2/users/me";

    private readonly JsonSerializerOptions _jsonOptions = new()
    {
        PropertyNameCaseInsensitive = false,
        PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
        WriteIndented = false
    };

    public async Task<Result<ActiveUserDto, ApplicationFailure>> GetMyUserAsync(CancellationToken cancellationToken)
    {
        var bearerToken = httpContextAccessor.HttpContext?.Request.Headers["Authorization"]
           .ToString()
           .Split(' ')
           .Last();

        if (string.IsNullOrWhiteSpace(bearerToken))
        {
            return Result.Failure<ActiveUserDto, ApplicationFailure>(new ApplicationFailure(
                ApplicationErrorType.DataApiIntegrationError,
                "No Bearer token found in the current request."));
        }

        var request = new HttpRequestMessage(HttpMethod.Get, GetMyUserEndpoint);
        request.Headers.Authorization = new AuthenticationHeaderValue("Bearer", bearerToken);

        var result = await client.SendAsync(request, cancellationToken);
        var content = await result.Content.ReadAsStringAsync(cancellationToken);

        if (!result.IsSuccessStatusCode)
        {
            return Result.Failure<ActiveUserDto, ApplicationFailure>(new ApplicationFailure(
                ApplicationErrorType.DataApiIntegrationError,
                $"Failed to get user data from X Data API. Status code: {result.StatusCode}, Response: {content}"));
        }

        var deserialized = JsonSerializer.Deserialize<ActiveUserDto>(content, _jsonOptions);

        if (deserialized is null)
        {
            return Result.Failure<ActiveUserDto, ApplicationFailure>(new ApplicationFailure(
                ApplicationErrorType.DataApiIntegrationError,
                "Failed to deserialize user data from X Data API."));
        }

        return deserialized;
    }
}