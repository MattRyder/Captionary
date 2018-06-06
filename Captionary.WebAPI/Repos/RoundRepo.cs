using System;
using System.Threading.Tasks;
using Captionary.Models.Concrete;
using StackExchange.Redis.Extensions.Core;

namespace Captionary.WebAPI.Repos
{
    public class RoundRepo : IRepo<Round>
    {
        private readonly string _cacheKeyPrefix = "Round_";
        private readonly ICacheClient _redis;

        public RoundRepo(ICacheClient redis)
        {
            this._redis = redis;
        }

        public void Delete(Round entity)
        {
            throw new System.NotImplementedException();
        }

        public Task<Round> FindAsync(string id)
        {
            return _redis.GetAsync<Round>(id);
        }

        public async Task<bool> SaveAsync(Round entity)
        {
            entity.ID = string.IsNullOrEmpty(entity.ID) ?
                Guid.NewGuid().ToString() : entity.ID;

            return await _redis.AddAsync(_cacheKeyPrefix + entity.ID, entity);
        }
    }
}