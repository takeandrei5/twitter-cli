using Collector.Core.UseCases.Creator.Queries.GetFollowedCreators;
using MediatR;
using Microsoft.AspNetCore.Mvc;

namespace Collector.API.Controllers;

[ApiController]
[Route("api/[controller]")]
public sealed class CreatorsController(ISender sender) : ControllerBase
{
    [HttpGet("followed")]
    public async Task<IResult> GetFollowedCreatorsQuery(CancellationToken cancellationToken)
    {
        var result = await sender.Send(new GetFollowedCreatorsQuery(), cancellationToken);

        return Results.Ok(result);
    }
}