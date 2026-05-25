using System.Globalization;
using System.Security.Claims;
using Collector.Core.Ports.X;

namespace Collector.API.Middlewares;

public sealed class HydrateCurrentUserMiddleware(IXDataApiClient dataApiClient) : IMiddleware
{
    public async Task InvokeAsync(HttpContext context, RequestDelegate next)
    {
        var myUser = await dataApiClient.GetMyUserAsync(context.RequestAborted);

        if (myUser.IsFailure)
        {
            context.Response.StatusCode = StatusCodes.Status500InternalServerError;
            return;
        }

        var claims = new[]
        {
            new Claim(ClaimTypes.NameIdentifier, myUser.Value.Id.ToString(CultureInfo.InvariantCulture)),
            new Claim(ClaimTypes.Name, myUser.Value.Name),
            new Claim(ClaimTypes.Actor, myUser.Value.Username)
        };

        var identity = new ClaimsIdentity(claims, "X");
        context.User = new ClaimsPrincipal(identity);

        await next(context);
    }
}