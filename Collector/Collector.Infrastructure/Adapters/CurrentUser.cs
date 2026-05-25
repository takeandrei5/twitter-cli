using System.Security.Claims;
using Collector.Core.Ports;
using Microsoft.AspNetCore.Http;

namespace Collector.Infrastructure.Adapters;

public sealed class CurrentUser(IHttpContextAccessor httpContextAccessor) : ICurrentUser
{
    private ClaimsPrincipal? Principal => httpContextAccessor.HttpContext?.User;

    public long Id
    {
        get
        {
            var value = Principal?.FindFirstValue(ClaimTypes.NameIdentifier);

            ArgumentException.ThrowIfNullOrWhiteSpace(value);

            if (long.TryParse(value, out var result))
            {
                return result;
            }

            throw new ArgumentException("Could not parse the user ID");
        }
    }

    public string Name
    {
        get
        {
            var value = Principal?.FindFirstValue(ClaimTypes.Name);

            ArgumentException.ThrowIfNullOrWhiteSpace(value);

            return value;
        }
    }

    public string Username
    {
        get
        {
            var value = Principal?.FindFirstValue(ClaimTypes.Actor);

            ArgumentException.ThrowIfNullOrWhiteSpace(value);

            return value;
        }
    }
}