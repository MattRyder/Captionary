using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Captionary.Models.Concrete;
using Captionary.WebAPI.Hubs;
using Captionary.WebAPI.Repos;
using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Hosting;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Options;
using StackExchange.Redis;
using StackExchange.Redis.Extensions.Core;
using StackExchange.Redis.Extensions.Core.Configuration;
using StackExchange.Redis.Extensions.Protobuf;

namespace Captionary.WebAPI
{
    public class Startup
    {
        public Startup(IConfiguration configuration)
        {
            Configuration = configuration;
        }

        public IConfiguration Configuration { get; }

        // This method gets called by the runtime. Use this method to add services to the container.
        public void ConfigureServices(IServiceCollection services)
        {
            services.AddCors(options =>
                options.AddPolicy("CorsPolicy", p =>
                    p.WithOrigins("http://localhost:3000")
                    .AllowAnyMethod()
                    .AllowCredentials()
                    .AllowAnyHeader()));

            services.AddMvc();

            services.AddSingleton<IConnectionMultiplexer, ConnectionMultiplexer>(
                provider => ConnectionMultiplexer.Connect("127.0.0.1"));

            var redisConfiguration = new RedisConfiguration()
            {
                AbortOnConnectFail = true,
                Hosts = new RedisHost[]
                {
                new RedisHost() { Host = "127.0.0.1", Port = 6379 },
                },
                AllowAdmin = true,
                ConnectTimeout = 3000,
                Database = 0,
                ServerEnumerationStrategy = new ServerEnumerationStrategy()
                {
                Mode = ServerEnumerationStrategy.ModeOptions.All,
                TargetRole = ServerEnumerationStrategy.TargetRoleOptions.Any,
                UnreachableServerAction = ServerEnumerationStrategy.UnreachableServerActionOptions.Throw
                }
            };

            services.AddSingleton(redisConfiguration);

            var serializer = new ProtobufSerializer();
            var redisClient = new StackExchangeRedisCacheClient(serializer, redisConfiguration);

            services.AddSingleton<ISerializer, ProtobufSerializer>(p => serializer);
            services.AddSingleton<ICacheClient, StackExchangeRedisCacheClient>(p => redisClient);

            services.AddTransient<IRepo<Room>, RoomRepo>();
            services.AddTransient<IRepo<Player>, PlayerRepo>();

            services.AddDistributedRedisCache((options) =>
            {
                options.Configuration = "localhost";
                options.InstanceName = "StagingCaptionaryInstance";
            });

            services.AddSignalR();
        }

        // This method gets called by the runtime. Use this method to configure the HTTP request pipeline.
        public void Configure(IApplicationBuilder app, IHostingEnvironment env)
        {
            if (env.IsDevelopment())
            {
                app.UseDeveloperExceptionPage();
            }

            app.UseCors("CorsPolicy");

            app.UseSignalR(routes =>
            {
                routes.MapHub<GameHub>("/game");
            });

            app.UseMvc();
        }
    }
}