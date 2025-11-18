import lemonade from "lemonadejs";
import { Component, SSE } from "shared/lib";

function LogInPage() {
    /*
<script>
    // Toggle password visibility
    document.getElementById('togglePassword').addEventListener('click', function() {
        const passwordInput = document.getElementById('password');
        const icon = this.querySelector('i');
        
        if (passwordInput.type === 'password') {
            passwordInput.type = 'text';
            icon.classList.remove('fa-eye');
            icon.classList.add('fa-eye-slash');
        } else {
            passwordInput.type = 'password';
            icon.classList.remove('fa-eye-slash');
            icon.classList.add('fa-eye');
        }
    });

    // Form submission
    document.getElementById('signinForm').addEventListener('submit', function(e) {
        e.preventDefault();
        
        // Get form values
        const username = document.getElementById('username').value;
        const password = document.getElementById('password').value;
        
        // In a real application, you would send this data to your server
        console.log('Admin sign in attempt:', { username, password });
        
        // Show a success message (in a real app, you'd handle authentication properly)
        alert('Admin authentication would be implemented here!');
    });
</script>
        *
        *
        */

    return (render: any) => render`
        <article class="flex items-center justify-center min-h-screen p-4">
    <div class="w-full max-w-md">
        <div class="text-center mb-10">
            <div class="flex justify-center items-center mb-6">
                <div class="w-14 h-14 rounded-xl bg-slate-800 flex items-center justify-center mr-3 border border-slate-700">
                    <i class="fas fa-gamepad text-blue-500 text-2xl"></i>
                </div>
                <h1 class="text-3xl font-bold text-white">VEIN</h1>
            </div>
            <p class="text-slate-400">Admin Panel Access</p>
        </div>

        <div class="bg-slate-800 rounded-xl border border-slate-700 p-8 shadow-lg">
            <form action="/sign-in" method="POST" id="signinForm" class="space-y-6">
                <div>
                    <label for="username" class="block text-sm font-medium text-slate-300 mb-2">Username</label>
                    <div class="relative">
                        <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                            <i class="fas fa-user text-slate-500"></i>
                        </div>
                        <input 
                            type="text" 
                            id="username" 
                            name="login" 
                            required 
                            class="input-focus bg-slate-900 border border-slate-700 text-white text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full pl-10 p-3.5 transition duration-200"
                            placeholder="Enter your username"
                        >
                    </div>
                </div>

                <div>
                    <label for="password" class="block text-sm font-medium text-slate-300 mb-2">Password</label>
                    <div class="relative">
                        <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                            <i class="fas fa-lock text-slate-500"></i>
                        </div>
                        <input 
                            type="password" 
                            id="password" 
                            name="password" 
                            required 
                            class="input-focus bg-slate-900 border border-slate-700 text-white text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full pl-10 p-3.5 transition duration-200"
                            placeholder="Enter your password"
                        >
                        <button type="button" id="togglePassword" class="absolute inset-y-0 right-0 pr-3 flex items-center">
                            <i class="fas fa-eye text-slate-500 hover:text-slate-300 transition duration-200"></i>
                        </button>
                    </div>
                </div>

                <button 
                    type="submit" 
                    class="w-full text-white bg-blue-600 hover:bg-blue-700 focus:ring-4 focus:outline-none focus:ring-blue-800 font-medium rounded-lg text-sm px-5 py-3.5 text-center transition duration-200"
                >
                    Sign In
                </button>
            </form>
        </div>

        <div class="mt-8 text-center text-xs text-slate-500">
            <p>&copy; 2023 Vein Game Dashboard. Restricted Access.</p>
        </div>
    </div>

</article>
        `;
}

lemonade.createWebComponent('login-page', LogInPage, { prefix: 'vein' });
