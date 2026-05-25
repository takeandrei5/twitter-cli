using Collector.Core.Entities;
using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata.Builders;

namespace Collector.Infrastructure.EntityConfigurations;

public sealed class UserConfiguration : IEntityTypeConfiguration<User>
{
    public void Configure(EntityTypeBuilder<User> builder)
    {
        builder.HasKey(u => u.Id);

        builder.Property(u => u.Id)
           .ValueGeneratedOnAdd();

        builder.Property(u => u.Username)
           .IsRequired()
           .HasMaxLength(100);

        

        builder.HasIndex(u => u.Username)
           .IsUnique();
    }
}