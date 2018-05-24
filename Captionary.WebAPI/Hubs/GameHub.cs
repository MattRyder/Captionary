using System;
using System.Collections.Concurrent;
using Microsoft.Extensions.Caching.Distributed;
using System.Collections.Generic;
using System.Threading.Tasks;
using Microsoft.AspNetCore.SignalR;
using Newtonsoft.Json.Linq;

namespace Captionary.WebAPI.Hubs {

    public class GameHub : Hub {
        
        private readonly IDistributedCache _cache;

        public GameHub (IDistributedCache cache) {
            this._cache = cache;
        }

        public override async Task OnConnectedAsync () {
            Console.WriteLine ("User connected to Captionary. CID: " + Context.ConnectionId);
            await Task.CompletedTask;
        }

        public override async Task OnDisconnectedAsync (Exception exception) {
            var disconnectedUserId = await _cache.GetStringAsync(Context.ConnectionId);

            Console.WriteLine (disconnectedUserId + "("+Context.ConnectionId+") disconnected from Captionary.");

            await _cache.RemoveAsync(Context.ConnectionId);
        }

        public async Task PlayerLogin (string playerName, int roomId) {
            Console.WriteLine ("Connection " + Context.ConnectionId + " logged in as: " + playerName);
            if(roomId > 0)
            {
                Console.WriteLine("Requesting access to Room "+ roomId);
            }

            await _cache.SetStringAsync(Context.ConnectionId, playerName);

            await Clients.Caller.SendAsync("JoinGame", roomId);
            await Clients.Others.SendAsync("PlayerConnected", playerName);
        }

        public async Task JoinRoomAsync () {
            await Groups.AddToGroupAsync (Context.ConnectionId, "Room1");
        }

        public async Task LeaveRoomAsync () {
            await Groups.RemoveFromGroupAsync (Context.ConnectionId, "Room1");
        }

        public async Task SendMessage (JObject message) {
            var playerName = await _cache.GetStringAsync(Context.ConnectionId);
            if(String.IsNullOrEmpty(playerName)) {
                return;
            }

            Console.WriteLine(playerName + "("+Context.ConnectionId+ ") says: " + message["message"]);
            
            message["senderId"] = 0;
            await Clients.Caller.SendAsync("ReceiveMessage", message);

            message["senderId"] = Context.ConnectionId;
            await Clients.Others.SendAsync("ReceiveMessage", message);
            
            
        }
    }
}