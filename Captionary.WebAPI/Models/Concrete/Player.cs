using Captionary.Models.Abstract;

namespace Captionary.Models.Concrete
{
    public class Player : IPlayer
    {
        string id;
        string name;

        public string ID
        {
            get { return id; }
            set { this.id = value; }
        }

        public string Name
        {
            get { return name; }
            set { this.name = value; }
        }
    }
}