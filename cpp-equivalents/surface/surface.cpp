#include <algorithm>
#include <iostream>

class Dummy {
	public:
		void destroy_surface(Dummy other) {
			std::cout << "Destroying dummy" << std::endl; 
		}
};
typedef Dummy Loader;
typedef Dummy SurfaceKHR;

class Window {
	public:
		Window(int a) : a(a) {}
		Window(Window&& move) {
			std::cout << "Moving window " << move.a << std::endl;
			this->a = move.a;
			move.a = -move.a;
		}

		~Window() {
			std::cout << "Destroying window " << this->a << std::endl;
		}

	// private:
		int a;
};

template<class W>
class Surface {
	public:
		Surface(W window) : window(std::move(window)) {}
		
		~Surface() {
			std::cout << "Destroying surface with window " << this->window.a << std::endl;

			this->loader.destroy_surface(this->surface);
		}

		W destroy_without_window() {
			return std::move(this->window);
		}

	// private:
		W window;

		Loader loader;
		SurfaceKHR surface;
};

int main() {
	std::cout << ">> Moving out" << std::endl;
	{
		Window original(1);
		std::cout << "original " << original.a << std::endl << std::endl;

		Surface<Window> surface(std::move(original));
		std::cout << "original " << original.a << std::endl;
		std::cout << "in surface " << surface.window.a << std::endl << std::endl;

		Window moved = surface.destroy_without_window();

		std::cout << "original " << original.a << std::endl;
		std::cout << "in surface " << surface.window.a << std::endl;
		std::cout << "moved out " << moved.a << std::endl << std::endl;
	}

	std::cout << std::endl << ">> Not moving out" << std::endl;
	{
		Window original(2);
		std::cout << "original " << original.a << std::endl << std::endl;

		Surface<Window> surface(std::move(original));
		std::cout << "original " << original.a << std::endl;
		std::cout << "in surface " << surface.window.a << std::endl << std::endl;
	}
}