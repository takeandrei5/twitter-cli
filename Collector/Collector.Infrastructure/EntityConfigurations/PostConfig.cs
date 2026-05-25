using Collector.Core.Entities;
using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata.Builders;

namespace Collector.Infrastructure.EntityConfigurations;

public sealed class PostConfig : IEntityTypeConfiguration<Post>
{
    public void Configure(EntityTypeBuilder<Post> builder)
    {
        builder.HasKey(p => p.Id);

        builder.Property(p => p.Id)
           .ValueGeneratedOnAdd();

        builder.Property(p => p.Content)
           .IsRequired()
           .HasMaxLength(280);

        builder.Property(p => p.ImageUrl)
           .HasMaxLength(2048);

        builder.HasOne(p => p.Creator)
           .WithMany()
           .HasForeignKey(p => p.CreatorId)
           .OnDelete(DeleteBehavior.Cascade);
    }
}