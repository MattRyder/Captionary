using System;
using System.Threading.Tasks;
using Captionary.Models.Concrete;
using StackExchange.Redis.Extensions.Core;

namespace Captionary.WebAPI.Repos
{
    public class PlayerRepo : IRepo<Player>
    {
        private readonly string _cacheKeyPrefix = "Player_";
        private readonly ICacheClient _redis;

        public PlayerRepo(ICacheClient redis)
        {
            this._redis = redis;
        }

        public void Delete(Player entity)
        {
            throw new System.NotImplementedException();
        }

        public Task<Player> FindAsync(string id)
        {
            return _redis.GetAsync<Player>(_cacheKeyPrefix + id);
        }

        public async Task<bool> SaveAsync(Player entity)
        {
            entity.ID = string.IsNullOrEmpty(entity.ID) ?
                Guid.NewGuid().ToString() : entity.ID;

            return await _redis.AddAsync(_cacheKeyPrefix + entity.ID, entity);
        }
    }
}