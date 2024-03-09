import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],
  // server: {
  //   // 配置服务器代理，实现跨域
  //   proxy: {
  //     //所有以 '/api'为前缀的接口都转向http://localhost:8000
  //     "/api": {
  //       target: "http://localhost:3000",
  //       changeOrigin: true,
  //       //去掉接口中的 '/api'以便和后端接口匹配
  //       rewrite: (path) => path.replace(/^\/api/, ""),
  //     },
  //   },
})



