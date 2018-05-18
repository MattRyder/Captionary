using System;
using System.Threading.Tasks;
using Microsoft.AspNetCore.SignalR;
using Newtonsoft.Json.Linq;

namespace Captionary.WebAPI.Hubs
{
    public class ChatHub : Hub
    {
        public override async Task OnConnectedAsync()
        {
            await Clients.All.SendAsync("PlayerConnected", "User1");
        }

        public override async Task OnDisconnectedAsync(Exception exception)
        {
            await Clients.All.SendAsync("PlayedDisconnected", "User1");
        }

        public async Task JoinRoomAsync()
        {
            await Groups.AddToGroupAsync(Context.ConnectionId, "Room1");
        }

        public async Task LeaveRoomAsync()
        {
            await Groups.RemoveFromGroupAsync(Context.ConnectionId, "Room1");
        }

        public async Task SendMessage(JObject message)
        {
            await Clients.All.SendAsync("ReceiveMessage", message);
        }
    }
}