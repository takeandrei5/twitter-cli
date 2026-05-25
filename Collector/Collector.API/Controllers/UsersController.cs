using Collector.API.Platform;
using Collector.Core.UseCases.Creator.Queries.GetFollowedCreators;
using Collector.Core.UseCases.User.Commands.CreateUser;
using MediatR;
using Microsoft.AspNetCore.Mvc;
using IResult = Microsoft.AspNetCore.Http.IResult;

namespace Collector.API.Controllers;

[ApiController]
[Route("api/[controller]")]
public sealed class UsersController(ISender sender) : ControllerBase
{
    [HttpPost]
    public async Task<IResult> Create(CancellationToken cancellationToken)
    {
        var result = await sender.Send(new CreateUserCommand(), cancellationToken);

        return result.ToApiResult();
    }
}