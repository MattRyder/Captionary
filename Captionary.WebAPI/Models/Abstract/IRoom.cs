using System;
using System.Collections.Generic;

namespace Captionary.Models.Abstract
{
    public interface IRoom
    {
        string ID { get; set; }

        IEnumerable<IPlayer> GetPlayers();

        bool AddPlayer(IPlayer player);

        bool RemovePlayer(IPlayer player);
    }
}