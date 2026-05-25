using Collector.Core.Errors;
using CSharpFunctionalExtensions;
using IResult = Microsoft.AspNetCore.Http.IResult;

namespace Collector.API.Platform;

public static class ResultHandler
{
    public static IResult ToApiResult(this UnitResult<ApplicationFailure> result)
    {
        return result.IsSuccess
            ? Results.Ok()
            : MapFailureToResult(result.Error);
    }

    public static IResult ToApiResult<T>(this Result<T, ApplicationFailure> result)
    {
        return result.IsSuccess
            ? Results.Ok(result.Value)
            : MapFailureToResult(result.Error);
    }

    private static IResult MapFailureToResult(ApplicationFailure failure)
    {
        var errorResponse = new { error = failure.ErrorType.ToString(), message = failure.Message };

        return failure.ErrorType switch
        {
            ApplicationErrorType.ConflictError => Results.Conflict(errorResponse),
            ApplicationErrorType.NotFoundError => Results.NotFound(errorResponse),
            ApplicationErrorType.ValidationError => Results.BadRequest(errorResponse),
            ApplicationErrorType.ExternalServiceError => Results.StatusCode(StatusCodes.Status502BadGateway),
            ApplicationErrorType.DataApiIntegrationError => Results.StatusCode(StatusCodes.Status502BadGateway),
            ApplicationErrorType.InternalServerError => Results.StatusCode(StatusCodes.Status500InternalServerError),
            _ => Results.StatusCode(StatusCodes.Status500InternalServerError)
        };
    }
}
