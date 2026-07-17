object AndroidClient {
    fun getNetworkType(context: android.content.Context): String {
        val connectivityManager = context.getSystemService(android.content.Context.CONNECTIVITY_SERVICE) as android.net.ConnectivityManager
        val networkInfo = connectivityManager.activeNetworkInfo
        return networkInfo?.typeName ?: "UNKNOWN"
    }
}