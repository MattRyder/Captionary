using System;
using System.Threading.Tasks;
using Captionary.Models.Concrete;
using StackExchange.Redis.Extensions.Core;

namespace Captionary.WebAPI.Repos
{
    public class RoomRepo : IRepo<Room>
    {
        private readonly string _cacheKeyPrefix = "Room_";
        private readonly ICacheClient _redis;

        public RoomRepo(ICacheClient redis)
        {
            this._redis = redis;
        }

        public void Delete(Room entity)
        {
            throw new System.NotImplementedException();
        }

        public Task<Room> FindAsync(string id)
        {
            return _redis.GetAsync<Room>(_cacheKeyPrefix + id);
        }

        public async Task<bool> SaveAsync(Room entity)
        {
            entity.ID = string.IsNullOrEmpty(entity.ID) ?
                Guid.NewGuid().ToString() : entity.ID;

            return await _redis.AddAsync(_cacheKeyPrefix + entity.ID, entity);
        }
    }
}