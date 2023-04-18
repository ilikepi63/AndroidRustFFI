package com.example.androidrustffi

import android.app.Service
import android.content.Intent
import android.os.IBinder
import android.util.Log
import android.widget.Toast

class RustService : Service() {

    private external fun startRuntime()

    fun rustFfiCallback() {
        println("ja no bru this looks like it works hey")
    }


    override fun onCreate() {

        Toast.makeText(this, "Going to do this...", Toast.LENGTH_SHORT).show()

        Log.i("Thread", "I am just testing the logging");

        Thread(Runnable {


            try{
                startRuntime()
            }catch(e: Exception){
                println("Error: ");
                println(e.toString())
            }

        }).start()


    }


    override fun onStartCommand(intent: Intent, flags: Int, startId: Int): Int {
        //Toast.makeText(this, "service starting", Toast.LENGTH_SHORT).show()

        return START_STICKY
    }

    override fun onBind(intent: Intent): IBinder? {
        return null
    }
    override fun onDestroy() {
        Toast.makeText(this, "service done", Toast.LENGTH_SHORT).show()
    }

    companion object {
        init {
            try{
                System.loadLibrary("rust")
            }catch(e:Exception){
                print(e.toString());
            }
        }

    }

}