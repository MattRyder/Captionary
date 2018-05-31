using Captionary.Models.Abstract;
using ProtoBuf;

namespace Captionary.Models.Concrete
{
    [ProtoContract]
    public class Player : IPlayer
    {
        string id;
        string name;

        [ProtoMember(1)]
        public string ID { 
            get { return id; }
            set { this.id = value; }
        }

        [ProtoMember(2)]
        public string Name {
            get { return name; }
            set { this.name = value; }
        }
    }
}