# PupsyEngine

## Создание Окна
* Создание ивент лупа окна через EventLoop::new()
* Создание нового окна через winit::window::WindowBuilder::new(), куда передаем размеры экрана, title и другие параметры

## Инициализация Vulkan

### 1. Создаем VkInstance
* Создаем ApplicationInfo
* Создаем InstanceCreateInfo
* создаем VkInstance
### 2. Создаем Surface по окну (пространство, куда собственно рендерится картинка)
* Создаем Surface (цепляем Hinstance и hwnd к vkInstance)
* Создаем SurfaceLoader
### 3. Создаем PhysicalDevice
* enumerate_physical_devices (итерируемся во всем девайсам, которые поддерживают Vulkan на машине)
    
    * get_physical_device_properties (характеристики карточки, версия api, имя и тд)
    * get_physical_device_queue_family_properties (получаем все доступные queue_families)
    * get_physical_device_features (получаем список всеъ фич карточки)

### 4. Создаем Device (logical device)
* Создаем DeviceQueueCreateInfo
* Создаем DeviceCreateInfo
* Создаем Device
### 5. Создаем GraphicsQueue
### 6. Создаем PresentQueue
### 7. Создаем Swapchain
* Создаем SwapchainCreateInfoKHR
* Создаем SwapchainLoader
* SwapchainLoadr.create_swapchain
### 8. Создаем PipelineLayout
* Создаем Shader Modules
    * Читаем байткод из скомпилированного шейдера
    * Создаем ShaderModuleCreateInfo
* Создаем Pipeline (https://vulkan-tutorial.com/Drawing_a_triangle/Graphics_pipeline_basics/Fixed_functions)
    * Vertex Input State (Описание аттрибутов (их оффсеты, размеры и тд), данные о самих вершинках)
    * Vertex Input Assembly Input State (Как именно должны восприниматься переданные вершинки, рисуются ли треугольники или линии, и каким образом)
    * Viewports and Scissors -> (ViewportState) (назначаем массивы viewport / scissors, которые будут обрезать конечный framebuffer)
    * (Optional) Dynamic States (некоторые элементы пайплайна можно изменять в рантайме в комманд буфере)
    * Rasterizer State (настройки растеризатора, тут настраивается face cull, depth cull, scissors cull, а также можно переопредилить, как рисуются треугольники, например сделать их линиями)
    * Multisampling State (Различные настройки для ультрабыстрого AA)
    * Depth Stencil State ()
    * Color Blending state (как смешиваются цвета в framebuffer после отработки фрагментного шейдера)
    * Pipeline Layout (сюда передаются global constants данные для шейдера)

        #### 8.1. Создаем Render Passes (https://vulkan-tutorial.com/Drawing_a_triangle/Graphics_pipeline_basics/Render_passes)

        * AttachmentDescription (Определяем поведение отображение во framebuffer после прохождния треугольника через пайплайн ())
        * Subpasses and Attachment references ()
            * SubpassDescription / AttachmentReference ()