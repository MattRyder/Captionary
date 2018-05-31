using System.Threading.Tasks;

namespace Captionary.WebAPI.Repos
{
    public interface IRepo<T>
    {
        Task<T> FindAsync(string id);

        Task<bool> SaveAsync(T entity);

        void Delete(T entity);
    }
}