/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

package hu.vmiklos.addr_osmify;

/**
 * Invokes osmify() on a thread.
 */
class Worker implements Runnable
{
    private Context context;

    public Worker(Context context) { this.context = context; }
    @Override public void run()
    {
        try
        {
            context.out = App.osmify(context.in);
        }
        catch (Exception e)
        {
            e.printStackTrace();
        }
    }
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
