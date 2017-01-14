package hu.vmiklos.tpconv;

import android.support.v7.app.AppCompatActivity;
import android.os.Bundle;
import android.view.View;
import android.widget.ArrayAdapter;
import android.widget.Button;
import android.widget.EditText;
import android.widget.Spinner;
import android.widget.TextView;

public class MainActivity extends AppCompatActivity {

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        String[] units = getUnitNames();

        Spinner from = (Spinner)findViewById(R.id.from);
        ArrayAdapter<String> fromArrayAdapter = new ArrayAdapter<String>(this, android.R.layout.simple_spinner_item, units);
        fromArrayAdapter.setDropDownViewResource(android.R.layout.simple_spinner_dropdown_item);
        from.setAdapter(fromArrayAdapter);

        Spinner to = (Spinner)findViewById(R.id.to);
        ArrayAdapter<String> toArrayAdapter = new ArrayAdapter<String>(this, android.R.layout.simple_spinner_item, units);
        toArrayAdapter.setDropDownViewResource(android.R.layout.simple_spinner_dropdown_item);
        to.setAdapter(fromArrayAdapter);
        to.setSelection(1);
    }

    public void convert(View v) {
        EditText amountWidget = (EditText)findViewById(R.id.amount);
        double amount = Double.parseDouble(amountWidget.getText().toString());
        Spinner fromWidget = (Spinner)findViewById(R.id.from);
        int from = fromWidget.getSelectedItemPosition();
        Spinner toWidget = (Spinner)findViewById(R.id.to);
        int to = toWidget.getSelectedItemPosition();

        double ret = convert(amount, from, to);

        TextView result = (TextView) findViewById(R.id.result);
        result.setText(String.valueOf(ret));
    }

    public native String[] getUnitNames();

    public native double convert(double amount, int from, int to);

    static {
        System.loadLibrary("native-lib");
    }
}
