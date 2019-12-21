/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

package hu.vmiklos.plees_tracker;

import android.content.Context;
import android.util.Log;

import androidx.room.Room;

import java.io.IOException;
import java.io.OutputStream;
import java.util.Date;
import java.util.List;

/**
 * Data model is the singleton shared state between the activity and the
 * service.
 */
public class DataModel
{
    private static final String TAG = "DataModel";
    private static final DataModel sDataModel = new DataModel();

    private Date mStart = null;
    private Date mStop = null;
    private Context mContext = null;
    private AppDatabase mDatabase = null;

    public static DataModel getDataModel() { return sDataModel; }

    private DataModel() {}

    void setStart(Date start) { mStart = start; }

    Date getStart() { return mStart; }

    void setStop(Date stop) { mStop = stop; }

    Date getStop() { return mStop; }

    void setContext(Context context) { mContext = context; }

    public AppDatabase getDatabase()
    {
        if (mDatabase == null)
        {
            mDatabase =
                Room.databaseBuilder(mContext, AppDatabase.class, "database")
                    .allowMainThreadQueries()
                    .build();
        }
        return mDatabase;
    }

    void storeSleep()
    {
        Sleep sleep = new Sleep();
        sleep.start = mStart.getTime();
        sleep.stop = mStop.getTime();
        getDatabase().sleepDao().insert(sleep);
    }

    String getSleepStat()
    {
        List<Sleep> sleeps = getDatabase().sleepDao().getAll();
        long sum = 0;
        for (Sleep sleep : sleeps)
        {
            long diff = sleep.stop - sleep.start;
            diff /= 1000;
            sum += diff;
        }
        int count = sleeps.size();
        if (count == 0)
        {
            return "";
        }
        String average = formatDuration(sum / count);
        String ret = "Average is " + average + " (" + count + " nights).";
        return ret;
    }

    void export(OutputStream os)
    {
        try
        {
            List<Sleep> sleeps = getDatabase().sleepDao().getAll();
            os.write("sid,start,stop\n".getBytes());
            for (Sleep sleep : sleeps)
            {
                StringBuilder row = new StringBuilder();
                row.append(sleep.sid);
                row.append(",");
                row.append(sleep.start);
                row.append(",");
                row.append(sleep.stop);
                row.append("\n");
                os.write(row.toString().getBytes());
            }
        }
        catch (IOException e)
        {
            Log.e(TAG, "export: write() failed");
        }
    }

    public static String formatDuration(long seconds)
    {
        return String.format("%d:%02d:%02d", seconds / 3600,
                             (seconds % 3600) / 60, seconds % 60);
    }
}
