using Collector.Core.Entities;
using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata.Builders;

namespace Collector.Infrastructure.EntityConfigurations;

public sealed class UserInterestConfiguration : IEntityTypeConfiguration<UserInterest>
{
    public void Configure(EntityTypeBuilder<UserInterest> builder)
    {
        builder.HasKey(userInterest => new { userInterest.UserId, userInterest.CreatorId });

        builder.HasOne(userInterest => userInterest.User)
            .WithMany(user => user.CreatorsFollowed)
            .HasForeignKey(userInterest => userInterest.UserId)
            .OnDelete(DeleteBehavior.Cascade);

        builder.HasOne(userInterest => userInterest.Creator)
            .WithMany(creator => creator.Followers)
            .HasForeignKey(userInterest => userInterest.CreatorId)
            .OnDelete(DeleteBehavior.Cascade);
    }
}